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
  set<string> vic;
  REP(i, 0, 2) {
    string s;
    cin >> s;
    vic.insert(s);
  }
  int n;
  cin >> n;
  REP(i, 0, n + 1) {
    if (i > 0) {
      string s, t;
      cin >> s >> t;
      vic.erase(s);
      vic.insert(t);
    }
    int pos = 0;
    for (auto v: vic) {
      cout << v << (pos == 1 ? "\n" : " ");
      pos += 1;
    }
  }
}
