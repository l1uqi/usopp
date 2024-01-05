export type Application = {
  name: string,
  soft_name: string,
  soft_install: string,
  soft_publisher: string,
  soft_icon_buffer: Array<number>,
  soft_icon_path: string,
  soft_run_path: string,
  url: string
}

export type SearchPaylod = {
  status: boolean;
  data: Array<Application>;
}
