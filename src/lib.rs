use essentials::wrapper::reason::Reason;

/// container which storged accessable data
pub trait Container 
{
    type Format;    /// storging struct type
    type Index;     /// type of accessing token
    type Agent;     /// agent type for more convenient in transfer

    fn access( &self, idx: Self::Index )-> Self::Format;
    fn export( &self )-> Self::Agent;
}


/// abliaty extension of data modifing for container
pub trait Modify: Container
{
    type Error; /// type of reason descriptor on failure in modify actions

    fn modify( &mut self, idx: Self::Index, dat: Self::Format )-> Reason<Self::Error>;
    fn append( &mut self, dat: Self::Format )-> Reason<Self::Error>;
}

