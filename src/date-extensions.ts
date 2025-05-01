/**
 * Adds astronomical Julian Date (JD) and Modified Julian Date (MJD) conversion methods to the JavaScript Date prototype.
 *
 * - Julian Date (JD): The continuous count of days since noon UTC on January 1, 4713 BC.
 * - Modified Julian Date (MJD): JD minus 2,400,000.5, commonly used in astronomy.
 *
 * These methods allow you to convert between JavaScript Date objects and their corresponding JD/MJD values.
 */

/**
 * Returns the Julian Date (JD) corresponding to this Date instance.
 * @returns {number} The Julian Date.
 */
Date.prototype.getJD = function (): number {
    return (this.getTime() / 86400000) + 2440587.5;
};

/**
 * Sets this Date instance to the given Julian Date (JD).
 * @param {number} jd - The Julian Date to set.
 */
Date.prototype.setJD = function (jd: number): void {
    this.setTime((jd - 2440587.5) * 86400000);
};

/**
 * Returns the Modified Julian Date (MJD) corresponding to this Date instance.
 * @returns {number} The Modified Julian Date.
 */
Date.prototype.getMJD = function (): number {
    return this.getJD() - 2400000.5;
};

/**
 * Sets this Date instance to the given Modified Julian Date (MJD).
 * @param {number} mjd - The Modified Julian Date to set.
 */
Date.prototype.setMJD = function (mjd: number): void {
    this.setJD(mjd + 2400000.5);
};