"use strict";

import * as net from "net";

import { Trace } from "vscode-jsonrpc";
import { window, workspace, commands, ExtensionContext, Uri } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  StreamInfo,
  Position as LSPosition,
  Location as LSLocation,
  ServerOptions,
} from "vscode-languageclient/node";

let lc: LanguageClient;

export function activate(context: ExtensionContext) {
  const serverOptions: ServerOptions = {
    command: context.asAbsolutePath("../target/release/locks.exe"),
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
