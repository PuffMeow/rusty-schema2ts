export type THairColor = 'color1' | 'color2' | 'color3';

export interface ISchema {
  firstName?: string;
  lastName?: string;
  age?: number;
  hairColor?: THairColor;
  obj?: IObj;
  arr?: IArr[];
}

export interface IObj {
  key1?: string;
  key2?: number;
  key3?: boolean;
}

export interface IArr {
  arr1?: string;
  arr2?: number;
  arr3?: IArr3[];
}

export interface IArr3 {
  enen1?: string;
  enen2?: number;
  enen3?: boolean;
  enen4?: any;
}
