use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    
    #[error("error al acceder al fichero: {error}")]
    AccesoError{
        #[from]
        #[source]
        error: ::std::io::Error
    },
}