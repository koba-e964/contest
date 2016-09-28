#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
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



int main(void){
  string s;
  int k;
  cin >> s >> k;
  REP(i, 0, s.length()) {
    int diff = s[i] - 'a';
    if (diff > 0) {
      diff = 26 - diff;
    }
    if (k < diff) {
      continue;
    }
    s[i] = 'a';
    k -= diff;
  }
  //shift the last letter by k
  int ch = s[s.length() - 1] - 'a';
  ch = (ch + k) % 26;
  s[s.length() - 1] = 'a' + ch;
  cout << s << endl;
}
