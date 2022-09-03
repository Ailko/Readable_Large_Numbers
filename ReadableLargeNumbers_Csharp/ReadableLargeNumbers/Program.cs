using System;
using System.Numerics;
using System.Resources;

namespace Readable_numbers
{
    class Program
    {
        //Strings to append for names
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

                int sectorcount = 0;    //Put generation of the number in {} to save memory space
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
                        value -= (value / BigInteger.Pow(10, (sectorcount - 1) * 3)) * BigInteger.Pow(10, (sectorcount - 1) * 3);   //Substract used value from number
                    }
                    sectorcount--;
                }

                if (value / 1000 != 0)      //Language being irregular, as usual *rolls eyes*
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
            int sectorTh = sectorcount / 1000;      //Split section that needs illinillion after it instead of illion
            int rest = sectorcount - sectorTh * 1000;       //"Regular" big numbers (max 10^2997)

            if (rest > 99)
            {
                result += Over100(rest);
            }
            else if (rest > 9)
            {
                result += Over10(rest);
            }
            else if (rest > 0)
            {
                result += Singles(rest);
            }

            if(rest != 0)
            {
                result += "illion";
            }

            if (sectorTh > 999)     //Honestly don't know why I added this, most programs refuse to generate numbers this big
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
            int sectorH = sectorcount / 100;        //Split hunderds of the sectors
            int sectorT = sectorcount / 10 - sectorH * 10;     //Splits Tens
            int sectorU = sectorcount - sectorT * 10 - sectorH * 100;  //Split Units

            if (sectorT > 0)
            {
                result += Over10(sectorT * 10 + sectorU);
                result += computelarger[1][sectorT - 1][computelarger[1][sectorT - 1].Length - 1];    //Add last letter (is excluded in Over10 because it is deleted for numbers under 10)
            }
            else if (sectorU > 0)
            {
                result += Irregularities(sectorU, 1);
            }

            result += computelarger[2][sectorH - 1];

            return result;
        }

        static string Over10(int sectorcount)
        {
            string result = "";
            int sectorT = sectorcount / 10;     //Split tens off
            int sectorU = sectorcount - sectorT * 10;       //Units

            if (sectorU != 0)
            {
                result += Irregularities(sectorU, 0);
            }

            if (sectorT != 0)
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

        static string Irregularities(int units, int noT)
        {
            string result = computelarger[0][units - 1];
            switch (units)        //Language irregularities intensify
            {
                case 3:
                case 6:
                    if (mods[noT][units - 1][0] != '\0')
                    {
                        result += mods[noT][units - 1][0];
                    }
                    break;
                case 7:
                case 9:
                    if (mods[noT][units - 1][1] != '\0')
                    {
                        result += mods[noT][units - 1][1];
                    }
                    break;
            }
            return result;
        }
    }
}