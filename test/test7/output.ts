export type THairColor2 = 'color1' | 'color2';

export type THairColor = 'color1' | 'color2' | 'color3';

export interface ISchema {
  hairColor?: THairColor;
  nestedObject?: INestedObject;
}

export interface INestedObject {
  hairColor?: THairColor2;
}
