#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}
