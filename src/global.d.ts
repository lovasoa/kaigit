/// <reference types="svelte" />

interface GitRepo {
    objects: string[];
}

interface Commit {
    type: "Commit";
    parents: string[];
}

interface Tree {
    type: "Tree";
    children: { name: string; id: string }[];
}

type GitObject = Commit | Tree;