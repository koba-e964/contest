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

string s[8];

int main(void){
  int h, w;
  cin >> h >> w;
  int c = 0;
  REP(i, 0, h) {
    cin >> s[i];
    REP(j, 0, w) {
      c += s[i][j] == '#';
    }
  }
  cout << (c == h + w - 1 ? "Possible" : "Impossible") << endl;
}
