export function insert(str:string, index, value) {
    return str.substring(0, index) + value + str.substring(index);
}