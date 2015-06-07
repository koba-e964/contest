#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;
int s[3];
bool ok(ll c) {
  ll q = 0;
  REP(j, 0, 3) {
    if (s[j] >= c) {
      q += (s[j] - c) / 2;
    } else {
      q += s[j] - c;
    }
  }
  return q >= 0;
}

int main(void){
  cin >> s[0] >> s[1] >> s[2];
  int c = 0;
  for (int i = 30; i >= 0; --i) {
    if (ok(c | 1 << i)) {
      c |= 1 << i;
    }
  }
  cout << c << endl;
}
