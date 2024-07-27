pub mod properties;

/// container which storged accessable data
pub trait Container
{
    type Units;     /// accessing unit of storging
    type Index;     /// type of accessing token
    type Agent;     /// agent type for more convenient in transfer

    fn access( &self, idx: &Self::Index )-> &Self::Units;
}

pub trait Serialize
{
    type Agent;

    fn export( &self )-> Self::Agent;
}