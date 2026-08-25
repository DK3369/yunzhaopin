export type ApiEnvelope<T = unknown> = {
  code: number
  key: string
  msg: string
  data: T | ''
}

export class ApiError extends Error {
  constructor(
    public code: number,
    public key: string,
    message: string,
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

export function unwrapEnvelope<T>(body: ApiEnvelope<T>): T {
  if (body.code === 200 && body.key === 'ok') {
    return (body.data === '' ? (undefined as T) : body.data) as T
  }
  throw new ApiError(body.code, body.key, body.msg)
}
