class Solution {
public:
    int romanToInt(string s) {
        vector<int> converter(128);
        int sum = 0;

        converter['I'] = 1;
        converter['V'] = 5;
        converter['X'] = 10;
        converter['L'] = 50;
        converter['C'] = 100;
        converter['D'] = 500;
        converter['M'] = 1000;

        for (int i = 0; i + 1 < s.length(); ++i)
            if (converter[s[i]] < converter[s[i + 1]])
                sum -= converter[s[i]];
            else
                sum += converter[s[i]];

        return sum + converter[s.back()];
    }
};