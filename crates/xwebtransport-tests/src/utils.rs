use xwebtransport_core::prelude::*;

pub async fn connect<Endpoint>(
    endpoint: Endpoint,
    url: &str,
    params: Endpoint::Params<'_>,
) -> Result<EndpointConnectConnectionFor<Endpoint>, xwebtransport_error::Connect<Endpoint>>
where
    Endpoint: xwebtransport_core::EndpointConnect,
    EndpointConnectConnectionFor<Endpoint>: xwebtransport_core::Connection,
{
    let connecting = endpoint
        .connect(url, params)
        .await
        .map_err(xwebtransport_error::Connect::Connect)?;

    let connection = connecting
        .wait()
        .await
        .map_err(xwebtransport_error::Connect::Connecting)?;

    Ok(connection)
}

pub async fn accept<Endpoint>(
    endpoint: Endpoint,
) -> Result<EndpointAcceptConnectionFor<Endpoint>, xwebtransport_error::Accept<Endpoint>>
where
    Endpoint: xwebtransport_core::EndpointAccept,
    EndpointAcceptConnectionFor<Endpoint>: xwebtransport_core::Connection,
{
    let connecting = endpoint
        .accept()
        .await
        .map_err(xwebtransport_error::Accept::Accept)?;
    let connection = connecting
        .wait()
        .await
        .map_err(xwebtransport_error::Accept::Connecting)?;

    Ok(connection)
}

pub async fn open_bi<Connection>(
    connection: Connection,
) -> Result<BiStreamsFor<Connection>, xwebtransport_error::OpenBi<Connection>>
where
    Connection: xwebtransport_core::OpenBiStream,
{
    let opening = connection
        .open_bi()
        .await
        .map_err(xwebtransport_error::OpenBi::Open)?;
    let streams = opening
        .wait_bi()
        .await
        .map_err(xwebtransport_error::OpenBi::Opening)?;

    Ok(streams)
}

pub async fn open_uni<Connection>(
    connection: Connection,
) -> Result<SendUniStreamFor<Connection>, xwebtransport_error::OpenUni<Connection>>
where
    Connection: xwebtransport_core::OpenUniStream,
{
    let opening = connection
        .open_uni()
        .await
        .map_err(xwebtransport_error::OpenUni::Open)?;
    let stream = opening
        .wait_uni()
        .await
        .map_err(xwebtransport_error::OpenUni::Opening)?;

    Ok(stream)
}
