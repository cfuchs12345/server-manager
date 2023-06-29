

export const mapValuesToArray = <K, V>(map: Map<K, V>): V[] =>  {
  return [...map.values()];
}

export const sortByNumericField = <T> (list : T[], getter: (val: T) => number ): T[] => {
  return list.sort( (a, b) => getter(a) - getter(b) );
}

export const sortByIpAddress = <T> (list: T[], getter: (val: T) => string ): T[] => {
  return list.sort( (a,b) => ipToNumber(getter(a)) - ipToNumber(getter(b)));
}

export const sortByString = <T> (list: T[], getter: (val: T) => string): T[] => {
  return list.sort( (a,b) => getter(a).localeCompare(getter(b)));
}

export const ipToNumber = (ipaddress: string): number => {
  return ipaddress
        .split('.')
        .map((num, idx) => parseInt(num) * Math.pow(2, (3 - idx) * 8))
        .reduce((a, v) => ((a += v), a), 0);
}


export const isType = <Type>(thing: any): thing is Type => true;
