use anchor_lang::error;

#[error]
pub enum ErrorCode {
    #[msg("Fee out of range")]
    FeeOutOfRangeError,
}