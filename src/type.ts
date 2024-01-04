export type Application = {
  name: string,
  display_name: string,
  run_path: string,
  install_location: string,
  icon_buffer: Array<number>,
  icon_path: string,
  url: string
}

export type SearchPaylod = {
  status: boolean;
  data: Array<Application>;
}
