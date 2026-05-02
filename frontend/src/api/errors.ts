export class NotFoundError extends Error {
  constructor() {
    super('Not Found');
    this.name = 'NotFoundError';
  }
}

export class UnprocessableContent extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'UnprocessableContent';
  }
}
