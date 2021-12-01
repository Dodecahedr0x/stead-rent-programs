import { expect } from "chai";
import {
  BN,
  setProvider,
  Provider,
  Program,
  workspace,
  web3,
} from "@project-serum/anchor";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  Token,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SteadRent } from "../target/types/stead_rent";
import { findAssociatedAddress } from "./helpers";

describe("stead-rent", () => {
  const provider = Provider.local();
  setProvider(provider);

  const program = workspace.SteadRent as Program<SteadRent>;

  const dao = Keypair.generate();
  const renter = Keypair.generate();
  const exhibitor = Keypair.generate();
  const buyer = Keypair.generate();
  let state: any;

  const collectionSize = 10;
  const initialBalance = new BN(10 ** 10);
  const feeAmount = new BN(500);

  const mintKeys: Token[] = Array(collectionSize).fill(undefined);
  const tokenAccounts: PublicKey[] = Array(collectionSize).fill(undefined);

  console.log(
    `Accounts:\n\tRenter: ${renter.publicKey.toString()}\n\tExhibitor: ${exhibitor.publicKey.toString()}`
  );

  const indexRented = 0;
  const indexDeposited = 1;

  it("Mints NFTs", async () => {
    await Promise.all(
      [renter, exhibitor, buyer].map(
        (keypair) =>
          new Promise(async (resolve) => {
            const airdrop = await provider.connection.requestAirdrop(
              keypair.publicKey,
              initialBalance.toNumber()
            );
            await provider.connection.confirmTransaction(airdrop);
            resolve(true);
          })
      )
    );

    const promises = [];
    for (let i = 0; i < collectionSize; i++) {
      promises.push(
        new Promise(async (resolve) => {
          const keypair = i === indexRented ? renter : exhibitor;

          mintKeys[i] = await Token.createMint(
            provider.connection,
            keypair,
            keypair.publicKey,
            null,
            0,
            TOKEN_PROGRAM_ID
          );

          tokenAccounts[i] = await mintKeys[i].createAccount(keypair.publicKey);

          await mintKeys[i].mintTo(tokenAccounts[i], keypair, [], 1);

          const accountInfo = await mintKeys[i].getAccountInfo(
            tokenAccounts[i]
          );
          expect(accountInfo.amount.toNumber()).to.equal(1);
          resolve(true);
        })
      );
    }
    await Promise.all(promises);
  });

  it("Initialize state", async () => {
    const [stateAddress, stateBump] = await PublicKey.findProgramAddress(
      [Buffer.from("state")],
      program.programId
    );
    state = stateAddress;

    await program.rpc.initState(
      stateBump,
      dao.publicKey,
      feeAmount.toNumber(),
      {
        accounts: {
          state: stateAddress,
          payer: provider.wallet.publicKey,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        },
      }
    );

    const s = await program.account.state.fetch(state)

    expect(s.feeEarner.toString()).to.equal(dao.publicKey.toString())
    expect(s.feeAmount).to.equal(feeAmount.toNumber())
  });

  it("Creates a new exhibition", async () => {
    const [exhibition, exhibitionBump] =
      await web3.PublicKey.findProgramAddress(
        [
          Buffer.from("exhibition", "utf8"),
          mintKeys[indexRented].publicKey.toBuffer(),
        ],
        program.programId
      );
    const [escrow, escrowBump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("escrow", "utf8"),
        mintKeys[indexRented].publicKey.toBuffer(),
      ],
      program.programId
    );
    const [exhibitionTokenKey, exhibitionTokenBump] =
      await web3.PublicKey.findProgramAddress(
        [
          Buffer.from("token_account", "utf8"),
          mintKeys[indexRented].publicKey.toBuffer(),
        ],
        program.programId
      );

    const bumps = {
      exhibition: exhibitionBump,
      escrow: escrowBump,
      exhibitionToken: exhibitionTokenBump,
    };

    console.log(
      `Accounts:\n\tExhibition: ${exhibition.toString()}\n\tExhibitor: ${escrow.toString()}`
    );

    await program.rpc.initExhibition(bumps, {
      accounts: {
        exhibition: exhibition,
        escrow: escrow,
        exhibitionTokenMint: mintKeys[indexRented].publicKey,
        exhibitionTokenAccount: exhibitionTokenKey,
        renter: renter.publicKey,
        renterAccount: tokenAccounts[indexRented],
        exhibitor: exhibitor.publicKey,
        payer: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      },
      signers: [renter],
    });

    const exhibitionToken = new Token(
      provider.connection,
      mintKeys[indexRented].publicKey,
      TOKEN_PROGRAM_ID,
      exhibitor
    );
    const renterAccount = await exhibitionToken.getAccountInfo(
      tokenAccounts[indexRented]
    );
    const escrowAccount = await exhibitionToken.getAccountInfo(
      exhibitionTokenKey
    );

    expect(renterAccount.amount.toNumber()).to.equal(new BN(0).toNumber());
    expect(escrowAccount.amount.toNumber()).to.equal(new BN(1).toNumber());
  });

  it("Deposits an item", async () => {
    const [exhibition] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("exhibition", "utf8"),
        mintKeys[indexRented].publicKey.toBuffer(),
      ],
      program.programId
    );
    const [escrow] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("escrow", "utf8"),
        mintKeys[indexRented].publicKey.toBuffer(),
      ],
      program.programId
    );
    const [depositedTokenKey, depositedTokenBump] =
      await web3.PublicKey.findProgramAddress(
        [
          Buffer.from("token_account", "utf8"),
          mintKeys[indexDeposited].publicKey.toBuffer(),
        ],
        program.programId
      );
    const [exhibitionItemKey, exhibitionItemBump] =
      await web3.PublicKey.findProgramAddress(
        [
          Buffer.from("item", "utf8"),
          exhibition.toBuffer(),
          mintKeys[indexDeposited].publicKey.toBuffer(),
        ],
        program.programId
      );

    const bumps = {
      item: exhibitionItemBump,
      tokenAccount: depositedTokenBump,
    };

    console.log(
      `Accounts:\n\tExhibition: ${exhibition.toString()}\n\tExhibitor: ${escrow.toString()}`
    );
    console.log(
      `\tDeposited Token: ${depositedTokenKey.toString()}\n\tItem: ${exhibitionItemKey.toString()}`
    );

    const definedPrice = new BN(10 ** 9);

    await program.rpc.depositToken(bumps, definedPrice, {
      accounts: {
        exhibition: exhibition,
        exhibitionItem: exhibitionItemKey,
        escrow: escrow,
        depositedTokenMint: mintKeys[indexDeposited].publicKey,
        depositedTokenAccount: depositedTokenKey,
        exhibitor: exhibitor.publicKey,
        exhibitorAccount: tokenAccounts[indexDeposited],
        payer: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
      },
      signers: [exhibitor],
    });

    const depositedToken = new Token(
      provider.connection,
      mintKeys[indexDeposited].publicKey,
      TOKEN_PROGRAM_ID,
      exhibitor
    );
    const exhibitorAccount = await depositedToken.getAccountInfo(
      tokenAccounts[indexDeposited]
    );
    const escrowAccount = await depositedToken.getAccountInfo(
      depositedTokenKey
    );

    expect(exhibitorAccount.amount.toNumber()).to.equal(new BN(0).toNumber());
    expect(escrowAccount.amount.toNumber()).to.equal(new BN(1).toNumber());

    const exhibitionItem = await program.account.exhibitionItem.fetch(
      exhibitionItemKey
    );
    expect(exhibitionItem.exhibition.toString()).to.equal(
      exhibition.toString()
    );
    expect(exhibitionItem.price.toNumber()).to.equal(definedPrice.toNumber());
  });

  it("Buys an item", async () => {
    const [exhibition] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("exhibition", "utf8"),
        mintKeys[indexRented].publicKey.toBuffer(),
      ],
      program.programId
    );
    const [escrow] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("escrow", "utf8"),
        mintKeys[indexRented].publicKey.toBuffer(),
      ],
      program.programId
    );
    const [depositedTokenKey, depositedTokenBump] =
      await web3.PublicKey.findProgramAddress(
        [
          Buffer.from("token_account", "utf8"),
          mintKeys[indexDeposited].publicKey.toBuffer(),
        ],
        program.programId
      );
    const [exhibitionItemKey, exhibitionItemBump] =
      await web3.PublicKey.findProgramAddress(
        [
          Buffer.from("item", "utf8"),
          exhibition.toBuffer(),
          mintKeys[indexDeposited].publicKey.toBuffer(),
        ],
        program.programId
      );

    const bumps = {
      item: exhibitionItemBump,
      tokenAccount: depositedTokenBump,
    };

    console.log(
      `Accounts:\n\tExhibition: ${exhibition.toString()}\n\tEscrow: ${escrow.toString()}`
    );
    console.log(
      `\tDeposited Token: ${depositedTokenKey.toString()}\n\tItem: ${exhibitionItemKey.toString()}`
    );

    const buyerAssociatedAccount = await mintKeys[
      indexDeposited
    ].getOrCreateAssociatedAccountInfo(buyer.publicKey);

    const definedPrice = new BN(10 ** 9);

    await program.rpc.buyToken(bumps, {
      accounts: {
        exhibition: exhibition,
        exhibitor: exhibitor.publicKey,
        exhibitionItem: exhibitionItemKey,
        escrow: escrow,
        depositedTokenMint: mintKeys[indexDeposited].publicKey,
        depositedTokenAccount: depositedTokenKey,
        buyer: buyer.publicKey,
        buyerAccount: buyerAssociatedAccount.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
      signers: [buyer],
    });

    const depositedToken = new Token(
      provider.connection,
      mintKeys[indexDeposited].publicKey,
      TOKEN_PROGRAM_ID,
      exhibitor
    );
    const buyerAccount = await depositedToken.getAccountInfo(
      buyerAssociatedAccount.address
    );
    const escrowAccount = await depositedToken.getAccountInfo(
      depositedTokenKey
    );

    expect(buyerAccount.amount.toNumber()).to.equal(new BN(1).toNumber());
    expect(escrowAccount.amount.toNumber()).to.equal(new BN(0).toNumber());

    const balance = await provider.connection.getBalance(buyer.publicKey);
    expect(balance <= initialBalance.sub(definedPrice).toNumber()).to.equal(
      true
    );
  });
});
