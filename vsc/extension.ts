"use strict";

import {
  workspace,
  ExtensionContext,
  commands,
  tasks,
  TaskDefinition,
  window,
  env,
  Uri,
} from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";
import { LocksTaskProvider } from "./locksTaskProvider";

let lc: LanguageClient;

export function activate(context: ExtensionContext) {
  const locksBinPath: string =
    workspace.getConfiguration("locks").get("binPath") ?? "locks";

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

  const runFileHandler = () => {
    const terminal = window.createTerminal(`locks run`);
    terminal.sendText(
      `${locksBinPath} run ${window.activeTextEditor.document.uri.path}`
    );
    terminal.show();
  };

  const disassembleFileHandler = () => {
    const terminal = window.createTerminal(`locks disassemble`);
    terminal.sendText(
      `${locksBinPath} disassemble ${window.activeTextEditor.document.uri.path}`
    );
    terminal.show();
  };

  context.subscriptions.push(
    commands.registerCommand(
      "locks.startLanguageServer",
      startLanguageServerHandler
    ),
    commands.registerCommand(
      "locks.stopLanguageServer",
      stopLanguageServerHandler
    ),
    commands.registerCommand("locks.runCurrentFile", runFileHandler),
    commands.registerCommand(
      "locks.disassembleCurrentFile",
      disassembleFileHandler
    ),
    commands.registerCommand("locks.openDocs", () => {
      env.openExternal(Uri.parse("https://kyleect.github.io/locks/#/docs"));
    })
  );

  tasks.registerTaskProvider("locks", new LocksTaskProvider());

  lc.start();
}

export function deactivate() {
  if (!lc) {
    return undefined;
  }

  return lc.stop();
}

interface LocksRunFileTaskDefinition extends TaskDefinition {
  /**
   * The locks file to run
   */
  file: string;
}
