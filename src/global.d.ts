/// <reference types="svelte" />

interface GitRepo {
    objects: string[];
    references: Reference[];
}

interface Reference {
    name: string;
    target: string;
    is_symbolic: boolean;
}

interface Commit {
    type: "Commit";
    parents: string[];
    author: string;
    message: string;
    tree:string;
}

interface Tree {
    type: "Tree";
    children: { name: string; id: string }[];
}

interface GitBlob {
    type: "Blob";
    size: number;
    short_contents: string;
}

interface Tag {
    type: "Tag";
    name: string;
    target: string;
}

interface Unknown {
    type: "Unknown";
}

type GitObject = Commit | Tree | GitBlob | Tag | Unknown;