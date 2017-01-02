#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

int main(void){
  int n;
  while ((cin >> n) && n) {
    vector<char> c(n);
    VI t(26, 0);
    REP(i, 0, n) {
      cin >> c[i];
      t[c[i] - 'A']++;
    }
    int dec = -1;
    char winner = 0;
    int ma = 0;
    REP(i, 0, 26) {
      ma = max(ma, t[i]);
    }
    REP(i, 0, 26) {
      if (t[i] == ma) {
	winner = winner ? '!' : 'A' + i;
      }
    }
    if (winner == '!') { winner = 0; }
    if (winner == 0) {
      cout << "TIE" << endl;
      continue;
    }
    fill(t.begin(), t.end(), 0);
    REP(i, 0, n) {
      if (dec >= 0) {
	break;
      }
      t[c[i] - 'A']++;
      int rest = n - i - 1;
      int sec = 0;
      REP(j, 0, 26) {
	if ('A' + j != winner) {
	  sec = max(sec, t[j]);
	}
      }
      if (sec + rest < t[winner - 'A']) {
	dec = i;
      }
    }
    cout << winner << " " << dec + 1 << endl;
  }
}
