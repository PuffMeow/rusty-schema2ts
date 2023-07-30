export type THairColor = "color1" | "color2" | "color3";

export interface ISchema {
  hairColor?: THairColor;
  nestedObject?: INestedObject;
}

export interface INestedObject {
  hairColor?: THairColor;
}
