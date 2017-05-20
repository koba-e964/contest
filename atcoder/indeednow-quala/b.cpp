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

bool check(string s) {
  string target = "indeednow";
  int cnt[26] = {};
  REP(i, 0, s.length()) {
    cnt[s[i] - 'a'] += 1;
  }
  REP(i, 0, target.length()) {
    cnt[target[i] - 'a'] -= 1;
  }
  REP(i, 0, 26) {
    if (cnt[i] != 0) {
      return false;
    }
  }
  return true;
}

int main(void){
  int n;
  cin >> n;
  REP(i, 0, n) {
    string s;
    cin >> s;
    cout << (check(s) ? "YES" : "NO") << endl;
  }
}
