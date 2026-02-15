export function extractError(err: unknown): string {
  if (typeof err === "string") return err
  if (err && typeof err === "object") {
    const values = Object.values(err)
    if (values.length > 0 && typeof values[0] === "string") return values[0]
  }
  return "Unknown error"
}
