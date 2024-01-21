use byteorder::LittleEndian;

type Slot = u64;
type Epoch = u64;
type Endianness = LittleEndian;

const GENESIS_SLOT: Slot = 0;
const GENESIS_EPOCH: Epoch = 0;
const FAR_FUTURE_EPOCH: Epoch = u64::MAX;
const BASE_REWARDS_PER_EPOCH: u64 = 4;
const DEPOSIT_CONTRACT_TREE_DEPTH: u64 = 2u64.pow(5);
const JUSTIFICATION_BITS_LENGTH: u64 = 4;
