/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface DownloadProgress {
  target: string
  downloaded: number
  total?: number | undefined | null
}
export function setupLog(): void
export class FileDownloader {
  constructor(emitter?: (...args: any[]) => any | undefined | null)
  downloadFile(url: string, filename: string): Promise<void>
}
