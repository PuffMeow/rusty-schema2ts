export type THairColor = 'color1' | 'color2' | 'color3';

/** Test */
export interface ITest {
  /** This is the first name */
  firstName: string;
  /** This is the last name */
  lastName: string;
  /** This is the age */
  age: number;
  /** This is the hair color */
  hairColor: THairColor;
  /** Object test */
  obj: IObj;
  /** Arr test Nested array items */
  arr: IArr[];
}

/** Object test */
export interface IObj {
  /** This is the key1 */
  key1: string;
  /** This is the key2 */
  key2: number;
  /** This is the key3 */
  key3: boolean;
}

/** Nested array items */
export interface IArr {
  /** This is the arr1 */
  arr1: string;
  /** This is the arr2 */
  arr2: number;
  /** Test arr3 Test nested arr3 items */
  arr3: IArr3[];
}

/** Test nested arr3 items */
export interface IArr3 {
  /** This is the enen1 */
  enen1: string;
  /** This is the enen2 */
  enen2: number;
  /** This is the enen3 */
  enen3: boolean;
}
