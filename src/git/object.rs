use super::sha::Sha;

enum Kind {
    Blob,
}

struct Object {
    kind: Kind,
    sha: Sha,
}
