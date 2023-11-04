import * as vscode from "vscode";

export class LocksTaskProvider implements vscode.TaskProvider {
  static TaskType = "locks";
  private task: vscode.Task | undefined;

  constructor() {
    this.task = new vscode.Task(
      {
        type: LocksTaskProvider.TaskType,
      },
      vscode.TaskScope.Workspace,
      "run",
      "locks"
    );
  }

  async provideTasks(token: vscode.CancellationToken): Promise<vscode.Task[]> {
    const tasks = [];
    tasks.push(this.task);
    return tasks;
  }

  resolveTask(
    task: vscode.Task,
    _token: vscode.CancellationToken
  ): vscode.Task | undefined {
    return new vscode.Task(
      task.definition,
      vscode.TaskScope.Workspace,
      task.name,
      task.source,
      new vscode.ShellExecution(
        `locks run ${this.getCurrentOpenFilePath()}`,
        {}
      )
    );
  }

  private getCurrentOpenFilePath(): string {
    return vscode.window.activeTextEditor.document.uri.path;
  }
}
