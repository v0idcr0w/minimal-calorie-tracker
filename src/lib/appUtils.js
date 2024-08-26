export function validateNumber(num) {
    return !isNaN(num) && Number(num) >= 0; 
}