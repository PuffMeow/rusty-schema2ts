type THairColor = 'color1' | 'color2' | 'color3';

interface ISchema {
  firstName?: string;
  lastName?: string;
  age?: number;
  hairColor?: THairColor;
  obj?: IObj;
  arr?: IArr[];
}

interface IObj {
  key1?: string;
  key2?: number;
  key3?: boolean;
}

interface IArr {
  arr1?: string;
  arr2?: number;
  arr3?: IArr3[];
}

interface IArr3 {
  enen1?: string;
  enen2?: number;
  enen3?: boolean;
  enen4?: any;
}
