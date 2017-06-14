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

const int N = 1234;
string s[N];

int main(void){
  int n, m, q;
  cin >> n >> m >> q;
  map<char, PI> tbl;
  REP(i, 0, n) {
    cin >> s[i];
    REP(j, 0, m) {
      if (s[i][j] != '*') {
	tbl[s[i][j]] = PI(i + 1, j + 1);
      }
    }
  }
  REP(i, 0, q) {
    char c;
    cin >> c;
    if (tbl.count(c)) {
      PI t = tbl[c];
      cout << t.first << " " << t.second << endl;
    } else {
      cout << "NA" << endl;
    }
  }
}
