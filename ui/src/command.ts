// - permissions
// - cooldown
// - description
// - handler

import { type Message, Role } from "./twitch";

export interface Command {
  allow?: Role;
  cooldown?: number;
  run(user: string, ...args: string[]): Promise<void>;
}

interface CommandState extends Command {
  allow: Role;
  cooldown: number;
  arity: number;
  lastRun: number;
}

export class CommandRegistry {
  commands = new Map<string, CommandState>();

  constructor(readonly prefix: string) {}

  async handle(message: Message) {
    let [prefix, commandName, ...args] = message.text.split(" ");
    if (prefix !== this.prefix) return;
    let command = this.commands.get(commandName);
    if (!command) {
      args = [commandName, ...args];
      command = this.commands.get("default");
      if (!command) return;
    }

    if (message.role() < command.allow) return;
    if (Date.now() - command.lastRun < command.cooldown) return;
    if (args.length < command.arity) return;
    command.run(message.user, ...args);
  }

  add<T extends Command>(name: string, command: T) {
    const { allow = Role.User, cooldown = 0, run, ...extra } = command;
    this.commands.set(name, {
      allow,
      cooldown,
      run,
      arity: command.run.length - 1,
      lastRun: Date.now(),
      ...extra,
    } as CommandState);
  }
}
