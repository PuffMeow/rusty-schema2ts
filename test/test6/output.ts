export type THairColor2 = 'color1' | 'color2';

export type THairColor = 'color1' | 'color2' | 'color3';

export interface ISchema {
  firstName?: string;
  lastName?: string;
  age?: number;
  hairColor?: THairColor;
  hairColor2?: THairColor2;
}
