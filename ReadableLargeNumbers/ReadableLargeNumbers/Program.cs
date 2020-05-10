using System;
using System.Numerics;

namespace Readable_numbers
{
    class Program
    {
        static public string[][] computelarger = new string[][] {
            new string[]{ "un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo", "nove" },
            new string[]{ "deci", "viginti", "triginta", "quadraginta", "quinquaginta", "sexaginta", "septuaginta", "octoginta", "nonaginta" },
            new string[]{ "cent", "ducent", "trecent", "quadringent", "quingent", "sescent", "septingenti", "octingent", "nongent" } };

        static public string[] smallerthan10 = new string[] { "m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non" };

        static public string[][] mods = new string[][] {
            new string[] { "\0n", "sm", "sn", "sn", "sn", "\0n", "\0n", "xm", "\0\0" },
            new string[] { "xn", "\0n", "sn", "sn", "sn", "\0n", "\0n", "xm", "\0\0" } };

        static void Main(string[] args)
        {
            BigInteger value = 0;

            while (value != -1)
            {
                Console.WriteLine("Please enter your number. (-1 to exit)");
                value = BigInteger.Parse(Console.ReadLine());
                Console.WriteLine();
                int sectorcount = 0;
                {
                    BigInteger valuecopy = value;
                    ulong length = 0;
                    while (valuecopy > 0)
                    {
                        length++;
                        valuecopy /= 10;
                    }
                    sectorcount = Convert.ToInt32((length + 2) / 3);
                }

                while (sectorcount > 2)
                {
                    if (value / BigInteger.Pow(10, (sectorcount - 1) * 3) != 0)
                    {
                        string concatstring = "";
                        if (sectorcount - 2 > 999)
                        {
                            concatstring = Over1000(sectorcount - 2);
                        }
                        else if (sectorcount - 2 > 99)
                        {
                            concatstring = Over100(sectorcount - 2);
                        }
                        else if (sectorcount - 2 > 9)
                        {
                            concatstring = Over10(sectorcount - 2);
                        }
                        else
                        {
                            concatstring = Singles(sectorcount - 2);
                        }
                        Console.WriteLine($"{value / BigInteger.Pow(10, (sectorcount - 1) * 3)} {concatstring}illion");
                        value -= (value / BigInteger.Pow(10, (sectorcount - 1) * 3)) * BigInteger.Pow(10, (sectorcount - 1) * 3);
                    }
                    sectorcount--;
                }

                if (value / 1000 != 0)
                {
                    Console.WriteLine($"{value / 1000} thousand");
                    value -= (value / 1000) * 1000;
                }
                if (value / 100 != 0)
                {
                    Console.Write($"{value / 100} hundred ");
                    value -= (value / 100) * 100;
                }
                if (value > 0)
                {
                    Console.Write($"and {value}");
                }
                Console.WriteLine("\n\n");
            }
        }

        static string Over1000(int sectorcount)
        {
            string result = "";
            int sectorTh = sectorcount / 1000;
            int rest = sectorcount - sectorTh * 1000;

            if (rest > 99)
            {
                result += Over100(rest) + "i";
            }
            else if (rest > 9)
            {
                result += Over10(rest) + computelarger[1][(rest - (rest - (rest / 10) * 10)) / 10 - 1].Substring(computelarger[1][(rest - (rest - (rest / 10) * 10)) / 10 - 1].Length - 1);
            }
            else if (rest > 0)
            {
                result += Singles(rest) + "illi";
            }

            if (sectorTh > 999)
            {
                result += Over1000(sectorTh);
            }
            else if (sectorTh > 99)
            {
                result += Over100(sectorTh);
            }
            else if (sectorTh > 9)
            {
                result += Over10(sectorTh);
            }
            else if (sectorTh > 0)
            {
                result += Singles(sectorTh);
            }

            result += "illin";

            return result;
        }

        static string Over100(int sectorcount)
        {
            string result = "";
            int sectorH = sectorcount / 100;
            int sectorTU = sectorcount - sectorH * 100;

            if (sectorTU > 9)
            {
                result += Over10(sectorTU);
                result += computelarger[1][(sectorTU - (sectorTU - (sectorTU / 10) * 10)) - 1][computelarger[1][(sectorTU - (sectorTU - (sectorTU / 10) * 10)) - 1].Length - 1];
            }
            else if (sectorTU > 0)
            {
                result += computelarger[0][sectorTU - 1];
                switch ((int)(sectorTU))
                {
                    case 3:
                    case 6:
                        if (mods[1][sectorTU - 1][0] != '\0')
                        {
                            result += mods[1][sectorTU - 1][0];
                        }
                        break;
                    case 7:
                    case 9:
                        if (mods[1][sectorTU - 1][1] != '\0')
                        {
                            result += mods[1][sectorTU - 1][1];
                        }
                        break;
                }
            }

            result += computelarger[2][sectorH - 1];

            return result;
        }

        static string Over10(int sectorcount)
        {
            string result = "";
            int sectorT = sectorcount / 10;
            int sectorU = sectorcount - sectorT * 10;

            if (sectorU != 0)
            {
                result += computelarger[0][sectorU - 1];
                switch ((int)(sectorU))
                {
                    case 3:
                    case 6:
                        if (mods[0][sectorT - 1][0] != '\0')
                        {
                            result += mods[0][sectorT - 1][0];
                        }
                        break;
                    case 7:
                    case 9:
                        if (mods[0][sectorT - 1][1] != '\0')
                        {
                            result += mods[0][sectorT - 1][1];
                        }
                        break;
                }
            }

            if (sectorcount - (sectorcount - (sectorcount / 10) * 10) != 0)
            {
                result += computelarger[1][sectorT - 1].Substring(0, computelarger[1][sectorT - 1].Length - 1);
            }

            return result;
        }

        static string Singles(int sectorcount)
        {
            string result = smallerthan10[sectorcount - 1];
            return result;
        }
    }
}