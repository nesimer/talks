import toString from "./toString.js";

describe('toString', () => {
  it('should format data as expected', async () => {
    const input = [
      {
        id: 2,
        firstName: 'L',
        lastName: 'M',
        sport: 'N',
        country: { id: 1, name: 'F' }
      },
      {
        id: 1,
        firstName: 'T',
        lastName: 'D',
        sport: 'J',
        country: { id: 1, name: 'F' }
      },
    ];

    expect.assertions(input.length);

    let index = 0;
    for await (const chunk of toString(input.values())) {
      const { country, sport, firstName, lastName } = input[index];
      expect(chunk).toBe(`[${country.name}] [${sport}] ${firstName} ${lastName.toUpperCase()}`)
      index++;
    }
  })
})