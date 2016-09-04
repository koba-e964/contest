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


int h, w;
const int H = 501;
string s[H];
string r[H], b[H];

int main(void){
  cin >> h >> w;
  REP(i, 0, h) {
    cin >> s[i];
    r[i] = b[i] = s[i];
  }
  REP(i, 1, h - 1) {
    if (i % 2 == 0) {
      REP(j, 1, w - 1) {
	r[i][j] = '#';
      }
    } else {
      REP(j, 1, w - 1) {
	b[i][j] = '#';
      }
    }
  }
  REP(i, 1, h - 1) {
    r[i][w - 1] = '#';
  }
  REP(i, 1, h - 1) {
    b[i][0] = '#';
  }
  REP(i, 0, h) {
    cout << r[i] << endl;
  }
  cout << endl;
  REP(i, 0, h) {
    cout << b[i] << endl;
  }
}
