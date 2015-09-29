#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

const int Q = 26;

VI to_freq(const string &s) {
	VI f(Q);
	REP(i, 0, s.length()) {
		f[s[i] - 'A']++;
	}
	return f;
}

int main(void){
	string a, b, c;
	cin >> a >> b >> c;
	VI fa = to_freq(a), fb = to_freq(b), fc = to_freq(c);
	VI inf(Q), sup(Q);
	int is = 0, ss = 0;
	REP(i, 0, Q) {
		inf[i] = max(0, fc[i] - fb[i]);
		sup[i] = min(fa[i], fc[i]);
		is += inf[i];
		ss += sup[i];
	}
	int n = a.length() / 2;
	cout << (is <= n && n <= ss ? "YES" : "NO") << endl;
}
