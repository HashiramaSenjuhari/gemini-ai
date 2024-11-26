use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Files {
    name: String,
    displayName: String,
    mimeType: String,
    sizeBytes: String,
    createTime: String,
    updateTime: String,
    expirationTime: String,
    sha256Hash: String,
    uri: String,
    state: String,
}

#[derive(Debug, Deserialize)]
pub struct JsonResponse {
    canditates: Vec<Main>,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    content: Vec<()>,
    finishReason: String,
    citationMetadata: CitationSource,
}

#[derive(Debug, Deserialize)]
pub struct CitationSource {
    citationSources: Vec<Source>,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    startIndex: u64,
    endIndex: u64,
    uri: String,
}
