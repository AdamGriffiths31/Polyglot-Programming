import { Opts } from './opts';
import path from 'path';

export enum Operation {
    Print,
    Add,
    Remove,
}

export type Config = {
    args: string[],
    operation: Operation,
    config: string,
    pwd: string,
}

function getPwd(opts: Opts): string {
    if (opts.pwd) {
        return opts.pwd;
    }

    return process.cwd();
}

function getConfig(opts: Opts): string {
    if (opts.config) {
        return opts.config;
    }

    const home = process.env["HOME"];
    const loc = process.env["XDG_CONFIG_HOME"] || home;
    if (!loc) {
        throw new Error("Could not find config location");
    }

    if (loc === home) {
        return path.join(loc, ".projector.json");
    }

    return path.join(loc, "projector", "config.json");
}

function getOperation(opts: Opts): Operation {
    if (!opts.args || opts.args.length === 0) {
        return Operation.Print;
    }

    switch (opts.args[0]) {
        case "add":
            return Operation.Add;
        case "remove":
            return Operation.Remove;
        default:
            throw new Error(`Unknown operation: ${opts.args[0]}`);
    }
}

function getArgs(opts: Opts): string[] {
    if (!opts.args || opts.args.length === 0) {
        return [];
    }

    const operation = getOperation(opts);
    if (operation === Operation.Print) {
        if (opts.args.length > 1) {
            throw new Error("Too many arguments");
        }
        return opts.args;
    }

    if (operation === Operation.Add) {
        if (opts.args.length !== 3) {
            throw new Error("Wrong number of arguments");
        }
        return opts.args.slice(1);
    }

    if (operation === Operation.Remove) {
        if (opts.args.length !== 2) {
            throw new Error("Wrong number of arguments");
        }
        return opts.args.slice(1);
    }

    throw new Error("Unknown operation");
}

export default function config(opts: Opts): Config {
    return {
        pwd: getPwd(opts),
        args: getArgs(opts),
        config: getConfig(opts),
        operation: getOperation(opts),
    };
}
