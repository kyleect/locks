"use strict";

import { workspace, ExtensionContext } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let lc: LanguageClient;

export function activate(context: ExtensionContext) {
  const locksBinPath: string =
    workspace.getConfiguration("locks").get("binPath") ??
    context.asAbsolutePath("../target/release/locks.exe");

  const serverOptions: ServerOptions = {
    command: locksBinPath,
    args: ["lsp"],
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      {
        language: "locks",
      },
    ],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/*.locks"),
    },
  };

  lc = new LanguageClient(
    "locks-language-server",
    serverOptions,
    clientOptions
  );

  lc.start();
}

export function deactivate() {
  if (!lc) {
    return undefined;
  }

  return lc.stop();
}
