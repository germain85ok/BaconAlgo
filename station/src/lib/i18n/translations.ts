// BaconAlgo Translation System
// FR/EN bilingual support

export const translations = {
	fr: {
		nav: {
			home: 'Accueil',
			dashboard: 'Tableau de bord',
			scanner: 'Scanner',
			signals: 'Signaux',
			portfolio: 'Portfolio',
			pricing: 'Tarification',
			academy: 'Académie',
			admin: 'Administration',
			login: 'Connexion',
			register: 'Inscription',
			logout: 'Déconnexion',
			profile: 'Profil'
		},
		pricing: {
			title: 'Choisissez votre plan',
			subtitle: 'Commencez gratuitement, améliorez quand vous êtes prêt. Tous les plans incluent un essai gratuit de 7 jours.',
			free: 'Gratuit',
			indicator: 'Indicateur',
			scanner: 'Scanner',
			station: 'Station',
			donation: 'Donation',
			monthly: 'Mensuel',
			yearly: 'Annuel',
			perMonth: '/mois',
			perYear: '/an',
			billedAnnually: 'facturé annuellement',
			save: 'Économisez 20%',
			mostPopular: 'Le plus populaire',
			getStarted: 'Commencer',
			subscribe: "S'abonner",
			paymentMethods: 'Méthodes de paiement acceptées',
			promoCode: 'Code promo',
			enterPromo: 'Entrez le code promo',
			apply: 'Appliquer',
			checking: 'Vérification...',
			faq: 'Questions Fréquentes',
			// Tier features
			freeTier: {
				signals: '3 signaux gratuits par jour',
				dashboard: 'Accès au dashboard de base',
				discord: 'Communauté Discord'
			},
			indicateurTier: {
				indicator: 'Indicateur BaconAlgo sur TradingView',
				alerts: 'Alertes watchlist',
				signals: 'Signaux illimités',
				support: 'Support email'
			},
			scannerTier: {
				everything: "Tout de l'Indicateur +",
				scanner: 'Scanner temps réel (10K+ instruments)',
				filters: 'Filtres V1-V5',
				confluence: 'Score de confluence',
				priority: 'Support prioritaire'
			},
			stationTier: {
				everything: 'Tout du Scanner +',
				autoTrade: 'Auto-exécution',
				brokers: 'Connexion broker (IBKR, Bitget)',
				overlay: 'Overlay streaming',
				backtesting: 'Backtesting illimité',
				fullApi: 'Accès API',
				vipSupport: 'Support VIP'
			}
		},
		signals: {
			confidence: 'Confiance',
			score: 'Score',
			grade: 'Grade',
			legendary: 'LÉGENDAIRE',
			epic: 'ÉPIQUE',
			rare: 'RARE',
			common: 'COMMUN',
			buy: 'ACHAT',
			sell: 'VENTE',
			loading: 'Chargement...',
			noSignals: 'Aucun signal disponible'
		},
		admin: {
			title: 'Administration',
			badge: 'ADMIN 🥓',
			overview: "Vue d'ensemble",
			users: 'Utilisateurs',
			revenue: 'Revenus',
			promoCodes: 'Codes promo',
			stats: 'Statistiques',
			logs: 'Logs système',
			signals: 'Signaux',
			// Stats
			totalUsers: 'Total utilisateurs',
			activeSubscriptions: 'Abonnements actifs',
			mrr: 'MRR',
			churnRate: 'Taux de désabonnement',
			newThisWeek: 'Nouveaux cette semaine',
			todaySignups: "Inscriptions aujourd'hui",
			activeToday: "Actifs aujourd'hui",
			revenueThisMonth: 'Revenus ce mois',
			todayRevenue: "Revenus aujourd'hui",
			// User management
			email: 'Email',
			username: "Nom d'utilisateur",
			plan: 'Plan',
			joined: 'Inscription',
			lastLogin: 'Dernière connexion',
			status: 'Statut',
			active: 'Actif',
			banned: 'Banni',
			changePlan: 'Modifier le plan',
			banUser: 'Bannir',
			unbanUser: 'Débannir',
			// Promo codes
			code: 'Code',
			type: 'Type',
			discount: 'Réduction',
			uses: 'Utilisations',
			maxUses: 'Max utilisations',
			createPromo: 'Créer un code promo',
			activate: 'Activer',
			deactivate: 'Désactiver',
			// Signal stats
			signalsPerDay: 'Signaux par jour',
			distribution: 'Distribution par grade',
			winRate: 'Taux de réussite',
			topInstruments: 'Instruments les plus scannés',
			// System logs
			recentErrors: 'Erreurs récentes',
			scannerPerf: 'Performance du scanner',
			brokerStatus: 'Statut connexion broker',
			apiUptime: "Disponibilité de l'API",
			// Quick actions
			quickActions: 'Actions rapides',
			createPromoAction: 'Créer Promo',
			sendAlert: 'Envoyer Alerte',
			pushSignal: 'Publier Signal',
			viewAll: 'Voir tout →',
			// Charts
			signupsChart: 'Inscriptions (7 derniers jours)',
			revenueChart: 'Revenus (30 derniers jours)',
			recentUsers: 'Utilisateurs récents'
		},
		common: {
			save: 'Sauvegarder',
			cancel: 'Annuler',
			delete: 'Supprimer',
			edit: 'Modifier',
			search: 'Rechercher',
			filter: 'Filtrer',
			noData: 'Aucune donnée',
			error: 'Erreur',
			success: 'Succès',
			loading: 'Chargement...',
			yes: 'Oui',
			no: 'Non',
			confirm: 'Confirmer'
		}
	},
	en: {
		nav: {
			home: 'Home',
			dashboard: 'Dashboard',
			scanner: 'Scanner',
			signals: 'Signals',
			portfolio: 'Portfolio',
			pricing: 'Pricing',
			academy: 'Academy',
			admin: 'Administration',
			login: 'Login',
			register: 'Register',
			logout: 'Logout',
			profile: 'Profile'
		},
		pricing: {
			title: 'Choose Your Plan',
			subtitle: 'Start free, upgrade when ready. All plans include 7-day free trial.',
			free: 'Free',
			indicator: 'Indicator',
			scanner: 'Scanner',
			station: 'Station',
			donation: 'Donation',
			monthly: 'Monthly',
			yearly: 'Yearly',
			perMonth: '/month',
			perYear: '/year',
			billedAnnually: 'billed annually',
			save: 'Save 20%',
			mostPopular: 'Most Popular',
			getStarted: 'Get Started',
			subscribe: 'Subscribe',
			paymentMethods: 'Accepted Payment Methods',
			promoCode: 'Promo Code',
			enterPromo: 'Enter promo code',
			apply: 'Apply',
			checking: 'Checking...',
			faq: 'Frequently Asked Questions',
			// Tier features
			freeTier: {
				signals: '3 free signals per day',
				dashboard: 'Basic dashboard access',
				discord: 'Discord community'
			},
			indicateurTier: {
				indicator: 'BaconAlgo indicator on TradingView',
				alerts: 'Watchlist alerts',
				signals: 'Unlimited signals',
				support: 'Email support'
			},
			scannerTier: {
				everything: 'Everything from Indicator +',
				scanner: 'Real-time scanner (10K+ instruments)',
				filters: 'V1-V5 filters',
				confluence: 'Confluence scoring',
				priority: 'Priority support'
			},
			stationTier: {
				everything: 'Everything from Scanner +',
				autoTrade: 'Auto-execution',
				brokers: 'Broker connection (IBKR, Bitget)',
				overlay: 'Stream overlay',
				backtesting: 'Unlimited backtesting',
				fullApi: 'API access',
				vipSupport: 'VIP support'
			}
		},
		signals: {
			confidence: 'Confidence',
			score: 'Score',
			grade: 'Grade',
			legendary: 'LEGENDARY',
			epic: 'EPIC',
			rare: 'RARE',
			common: 'COMMON',
			buy: 'BUY',
			sell: 'SELL',
			loading: 'Loading...',
			noSignals: 'No signals available'
		},
		admin: {
			title: 'Administration',
			badge: 'ADMIN 🥓',
			overview: 'Overview',
			users: 'Users',
			revenue: 'Revenue',
			promoCodes: 'Promo Codes',
			stats: 'Statistics',
			logs: 'System Logs',
			signals: 'Signals',
			// Stats
			totalUsers: 'Total Users',
			activeSubscriptions: 'Active Subscriptions',
			mrr: 'MRR',
			churnRate: 'Churn Rate',
			newThisWeek: 'New This Week',
			todaySignups: "Today's Signups",
			activeToday: 'Active Today',
			revenueThisMonth: 'Revenue This Month',
			todayRevenue: "Today's Revenue",
			// User management
			email: 'Email',
			username: 'Username',
			plan: 'Plan',
			joined: 'Joined',
			lastLogin: 'Last Login',
			status: 'Status',
			active: 'Active',
			banned: 'Banned',
			changePlan: 'Change Plan',
			banUser: 'Ban',
			unbanUser: 'Unban',
			// Promo codes
			code: 'Code',
			type: 'Type',
			discount: 'Discount',
			uses: 'Uses',
			maxUses: 'Max Uses',
			createPromo: 'Create Promo Code',
			activate: 'Activate',
			deactivate: 'Deactivate',
			// Signal stats
			signalsPerDay: 'Signals per day',
			distribution: 'Distribution by grade',
			winRate: 'Win rate',
			topInstruments: 'Most scanned instruments',
			// System logs
			recentErrors: 'Recent errors',
			scannerPerf: 'Scanner performance',
			brokerStatus: 'Broker connection status',
			apiUptime: 'API uptime',
			// Quick actions
			quickActions: 'Quick Actions',
			createPromoAction: 'Create Promo',
			sendAlert: 'Send Alert',
			pushSignal: 'Push Signal',
			viewAll: 'View All →',
			// Charts
			signupsChart: 'Signups (Last 7 Days)',
			revenueChart: 'Revenue (Last 30 Days)',
			recentUsers: 'Recent Users'
		},
		common: {
			save: 'Save',
			cancel: 'Cancel',
			delete: 'Delete',
			edit: 'Edit',
			search: 'Search',
			filter: 'Filter',
			noData: 'No data',
			error: 'Error',
			success: 'Success',
			loading: 'Loading...',
			yes: 'Yes',
			no: 'No',
			confirm: 'Confirm'
		}
	}
} as const;

export type Language = 'fr' | 'en';
export type TranslationKey = keyof typeof translations.en;
