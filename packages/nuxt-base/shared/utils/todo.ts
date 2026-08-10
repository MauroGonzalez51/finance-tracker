import process from "node:process";

export interface TodoOptions {
    message?: string;
    meta?: Record<string, unknown>;
    cause?: unknown;
}

class TodoError extends Error {
    readonly timestamp: Temporal.PlainDate;
    readonly pid: number;
    readonly environment: string;
    readonly meta?: Record<string, unknown>;

    constructor(options: TodoOptions = {}) {
        const { message = "Not Implemented", meta, cause } = options;

        super(message, { cause });

        this.name = "TodoError";
        this.timestamp = Temporal.Now.plainDateISO();
        this.pid = process.pid;
        this.environment = process.env.NODE_ENV ?? "development";
        this.meta = meta;

        Error.captureStackTrace(this, TodoError);
    }

    private getCallerLocation(): string | undefined {
        const line = this.stack
            ?.split("\n")
            .slice(1)
            .find((line) => !line.includes("TodoError"));

        return line?.trim();
    }

    override toString(): string {
        const location = this.getCallerLocation();

        return [
            `${this.name}: ${this.message}`,
            `Location: ${location ?? "unknown"}`,
            `Time: ${this.timestamp}`,
            `Environment: ${this.environment}`,
            this.meta && `Meta: ${JSON.stringify(this.meta, null, 4)}`,
            this.stack ?? "",
        ]
            .filter(Boolean)
            .join("\n");
    }
}

export function todo(options: string | TodoOptions = "Not Implemented") {
    if (typeof options === "string") {
        throw new TodoError({ message: options });
    }

    throw new TodoError(options);
}
