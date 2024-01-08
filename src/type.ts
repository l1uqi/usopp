export type Application = {
  name: string,
  text_name: string,
  r_type: string,
  r_install: string,
  r_publisher: string,
  r_icon_buffer: Array<number>,
  r_icon_path: string,
  r_exe_path: string,
  url: string
}

export type SearchPaylod = {
  status: boolean;
  data: Array<Application>;
}
