#include <algorithm>
#include <iomanip>
#include <iostream>
#include <string>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  bool eq = true;
  string pool;
  int conc = 0;
  string vowel = "aeiou";
  REP(i, 0, s.length()) {
    bool is_cons = find(vowel.begin(), vowel.end(), s[i]) == vowel.end();
    if (not is_cons) {
      eq = true;
      conc = 0;
      pool += s[i];
      continue;
    }
    if (conc >= 2 && not (eq && pool[pool.size() - 1] == s[i])) {
      cout << pool << " ";
      eq = true;
      pool = "";
      conc = 0;
    }
    if (conc >= 1 && pool[pool.size() - 1] != s[i]) {
      eq = false;
    }
    pool += s[i];
    conc += 1;
  }
  cout << pool << endl;
}
