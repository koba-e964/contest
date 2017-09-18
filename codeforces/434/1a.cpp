#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



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
      conc = 0;
      pool += s[i];
      continue;
    }
    if (conc >= 2 && not (eq && pool[0] == s[i])) {
      cout << pool << " ";
      eq = true;
      pool = "";
      conc = 0;
    }
    if (pool.size() >= 1 && pool[0] != s[i]) {
      eq = false;
    }
    pool += s[i];
    conc += 1;
  }
  cout << pool << endl;
}
