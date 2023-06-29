

export const mapValuesToArray = <K, V>(map: Map<K, V>): V[] =>  {
  return [...map.values()];
}

export const sortByNumericField = <T> (list : T[], getter: (val: T) => number ): T[] => {
  return list.sort( (a, b) => getter(a) - getter(b) );
}




export const isType = <Type>(thing: any): thing is Type => true;
