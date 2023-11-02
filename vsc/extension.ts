"use strict";

import { workspace, ExtensionContext, commands } from "vscode";
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
    outputChannelName: "locks",
  };

  lc = new LanguageClient(
    "locks-language-server",
    serverOptions,
    clientOptions
  );

  const startLanguageServerHandler = () => {
    console.log("Starting locks language server...");
    lc.start();
  };

  const stopLanguageServerHandler = () => {
    console.log("Stopping locks language server...");

    if (!lc) {
      return undefined;
    }

    return lc.stop();
  };

  context.subscriptions.push(
    commands.registerCommand(
      "locks.startLanguageServer",
      startLanguageServerHandler
    ),
    commands.registerCommand(
      "locks.stopLanguageServer",
      stopLanguageServerHandler
    )
  );

  lc.start();
}

export function deactivate() {
  if (!lc) {
    return undefined;
  }

  return lc.stop();
}
