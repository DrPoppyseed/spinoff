use strum::EnumIter;
/// An enum of all the available spinners.
/// Contains around 80+ spinners.
/// Each variant in this enum is assigned to a HashMap holding it's frames and interval count.
#[derive(Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum Spinners {
    Dots,
    Dots2,
    Dots3,
    Dots4,
    Dots5,
    Dots6,
    Dots7,
    Dots8,
    Dots9,
    Dots10,
    Dots11,
    Dots12,
    Dots8Bit,
    Line,
    Line2,
    Pipe,
    SimpleDots,
    SimpleDotsScrolling,
    Star,
    Star2,
    Flip,
    Hamburger,
    GrowVertical,
    GrowHorizontal,
    Balloon,
    Balloon2,
    Noise,
    Bounce,
    BoxBounce,
    BoxBounce2,
    Triangle,
    Arc,
    Circle,
    SquareCorners,
    CircleQuarters,
    CircleHalves,
    Squish,
    Toggle,
    Toggle2,
    Toggle3,
    Toggle4,
    Toggle5,
    Toggle6,
    Toggle7,
    Toggle8,
    Toggle9,
    Toggle10,
    Toggle11,
    Toggle12,
    Toggle13,
    Arrow,
    Arrow2,
    Arrow3,
    BouncingBar,
    BouncingBall,
    Smiley,
    Monkey,
    Hearts,
    Clock,
    Earth,
    Material,
    Moon,
    Runner,
    Pong,
    Shark,
    Dqpb,
    Weather,
    Christmas,
    Grenade,
    Point,
    Layer,
    BetaWave,
    FingerDance,
    FistBump,
    SoccerHeader,
    Mindblown,
    Speaker,
    OrangePulse,
    BluePulse,
    OrangeBluePulse,
    TimeTravel,
    Aesthetic,
}
