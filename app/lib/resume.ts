// biome-ignore-all lint/correctness/noNodejsModules: build-time data loading; runs on the server, never shipped to the client
import { readFileSync } from "node:fs";
import path from "node:path";
import process from "node:process";
import { parse } from "smol-toml";

interface ResumeData {
  roles: Role[];
  publications: Publication[];
  honours: Honour[];
}

export interface Role {
  from: string;
  to: string;
  org: string;
  loc: string;
  title: string;
  sub: string;
  body: string;
  tags: string[];
  featured?: boolean;
}

export interface Publication {
  authors: string;
  title: string;
  journal: string;
  year: number;
  doi?: string;
  arxiv?: string;
}

export interface Honour {
  year: number;
  award: string;
  org: string;
}

export function loadResume(): {
  roles: Role[];
  publications: Publication[];
  honours: Honour[];
} {
  const tomlPath = path.resolve(process.cwd(), "app/content/resume.toml");
  const raw = readFileSync(tomlPath, "utf-8");
  const data = parse(raw) as unknown as ResumeData;
  return {
    roles: data.roles,
    publications: data.publications,
    honours: data.honours,
  };
}
